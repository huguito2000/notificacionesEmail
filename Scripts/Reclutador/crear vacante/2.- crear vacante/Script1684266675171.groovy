import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import java.nio.file.WatchService as WatchService
import javax.xml.bind.annotation.XmlElementDecl.GLOBAL as GLOBAL
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.keyword.builtin.GetResponseStatusCodeKeyword as GetResponseStatusCodeKeyword
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import com.kms.katalon.entity.global.GlobalVariableEntity as GlobalVariableEntity
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonOutput as JsonOutput
import groovy.json.JsonSlurper as JsonSlurper
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject



puestos = ['Administración y gestión de empresas', 'Contabilidad y fiscalización', 'Finanzas", "banca y seguros', 'Negocios y administración, programas multidisciplinarios o generales'
    , 'Negocios y comercio', 'Ciencias políticas', 'Economía', 'Psicología', 'Sociología y antropología', 'Trabajo y atención social'
    , 'Ciencias de la información', 'Comunicación y periodismo', 'Criminología', 'Derecho', 'Ingeniería, manufactura y construcción'
    , 'Arquitectura y construcción', 'Arquitectura y urbanismo', 'Construcción e ingeniería civil', 'Ingeniería industrial'
    , 'mecánica', 'electrónica', 'tecnología', 'Electricidad', 'generación de energía', 'Electrónica', 'automatización', 'Ingeniería de vehículos de motor'
    , 'Ingeniería barcos', 'Ingeniería aeronaves', 'Ingeniería mecánica', 'Ingeniería electrónica', 'Ingeniería tecnología'
    , 'programas multidisciplinarios o generales', 'Ingeniería mecánica ', 'Ingeniería metalurgia', 'Ingeniería química'
    , 'Tecnología y protección del medio ambiente', 'Tecnologías de la información y comunicación', 'Manufacturas y procesos'
    , 'Industria de la alimentación', 'Manufacturas ', 'programas multidisciplinarios', 'Minería y extracción', 'Educación'
    , 'Formación docente', 'Bellas artes', 'Formación docente para educación básica, nivel preescolar', 'Formación docente para educación básica, nivel primaria'
    , 'Formación docente para educación básica, nivel secundaria', 'Formación docente para educación física, artística o tecnológica'
    , 'Formación docente para la enseñanza de asignaturas específicas', 'Formación docente para otros servicios educativos'
    , 'Formación docente, programas multidisciplinarios o generales', 'Educación', 'Ciencias de la educación', 'Didáctica, pedagogía y currículo'
    , 'Orientación y asesoría educativa', 'Ciencias exactas de la computación', 'Ciencias naturales', 'Biología y bioquímica'
    , 'Ciencias ambientales', 'Ciencias de la computación', 'Ciencias físicas', 'Ciencias químicas', 'Ciencias de la tierra'
    , 'Ciencias de la atmósfera', 'Física', 'Química', 'Matemáticas', 'estadística', 'Servicios personales', 'Deportes', 'Servicios de transporte'
    , 'humanidades', 'Artes', 'Diseño', 'Música', 'artes escénicas', 'Técnicas audiovisuales', 'producción de medios', 'Filosofía'
    , 'ética', 'Historia ', 'arqueologia', 'Lenguas extranjeras', 'Literatura', 'Salud', 'Enfermería y cuidados', 'Estomatología '
    , 'odontología', 'Medicina', 'Salud pública', 'Terapia y rehabilitación', 'Agronomía', 'veterinaria', 'silvicultura'
    , 'pesca', 'ganaderia']

Random rand = new Random()

int ranlist = rand.nextInt(puestos.size())

GlobalVariable.puesto = puestos.get(ranlist)

println(GlobalVariable.puesto)

WebUI.callTestCase(findTestCase('Reclutador/login/login'), [:], FailureHandling.STOP_ON_FAILURE)

println('Inicia el test crear vacante')

response = WS.sendRequest(findTestObject('Reclutador/crear vacantes/crear vacante'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

response = WS.verifyResponseStatusCode(response, 200)

