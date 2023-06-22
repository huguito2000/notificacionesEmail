import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Candidato/Registro/formulario'), [:], FailureHandling.STOP_ON_FAILURE)

salaryrequery = 0

println ("la fecha es")
date()

println ("el puesto es")
puestos()

println ("el tipo de puesto es")
tiposPuestos ()

println ("el nivel academico es")
academyNivel ()

response = WS.sendRequest(findTestObject('candidato/registro/11.- trabajo'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/12.- trabajo localidad'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/13.- localidad'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

Salario()
salaryrequery = salaryrequery + 1000
println(salaryrequery)
GlobalVariable.SalarioDeseado = salaryrequery

response = WS.sendRequest(findTestObject('candidato/registro/14.- sueldo'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/15.- CV'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/16.- carga CV'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 201)

response = WS.sendRequest(findTestObject('candidato/registro/16.- carga CV'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 201)

println ("la fecha es")
date()

response = WS.sendRequest(findTestObject('candidato/registro/17.- experencia'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

while(statusCode !=200) {
	
	println ("la fecha es")
	date()
	
	println ("el puesto es")
	puestos()
	
	println ("el tipo de puesto es")
	tiposPuestos ()
		
	response = WS.sendRequest(findTestObject('candidato/registro/17.- experencia'))

	statusCode = WS.getResponseStatusCode(response)

	println(statusCode)

	WebUI.delay(5)

}

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/18.- Educacion'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/19.- area'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/20.- habilidades duras'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

WebUI.openBrowser('https://yopmail.com/es/wm')

WebUI.setText(findTestObject('Generales/Cuenta Bloqueada/Campo Email'), GlobalVariable.email)

WebUI.sendKeys(findTestObject('Generales/Cuenta Bloqueada/Campo Email'), Keys.chord(Keys.ENTER))

WebUI.takeScreenshot('/Users/huguito/Desktop/notificaciones/Candidatos/registro.png')

WebUI.delay(5)

WebUI.closeBrowser()

def Salario() {
	salaryrequery= ((Math.random() * 30000) as int)
	}
	def date() {
		
		fecha = ["2004-12-12","2003-12-12","2002-12-12","2001-12-12","2000-12-12","1999-12-12","1998-12-12","1997-12-12","1996-12-12","1995-12-12",
			"1994-12-12","1993-12-12","1992-12-12","1991-12-12","1990-12-12","1989-12-12","1988-12-12","1987-12-12","1986-12-12","1985-12-12","1984-12-12",
			"1983-12-12","1982-12-12",,"1981-12-12","1980-12-12","1979-12-12","1978-12-12","1977-12-12","1976-12-12","1975-12-12","1974-12-12",
			"1973-12-12","1972-12-12","1971-12-12","1970-12-12","1969-12-12","1968-12-12","1967-12-12","1966-12-12","1965-12-12","1964-12-12",
			"1963-12-12","1962-12-12","1961-12-12","1960-12-12"]
		
		Random rand = new Random()
		
		int ranlist = rand.nextInt(fecha.size())
		
		GlobalVariable.fechaExp = fecha.get(ranlist)
		
		println(GlobalVariable.fechaExp)
	}
	
	def puestos() {
	
		puesto = ['administrador', 'Contador', 'financiero", "asegurador', 'administrador de negocios', 'programas multidisciplinarios o generales'
			, 'Operador comercial de transporte público', 'Mecánico de vehículos ligeros', 'Mecánico de motocicletas', 'psicologo', 'Sociologo',  'antropologo', 
			, 'periodista', 'Criminologo', 'Abogado', 'Ingeniero', 'manufacturero', 'Arquitecto', 'Arquitecto de urbanismo', 'ingeniero civil', 'Ingeniero industrial'
			, 'mecánica', 'electrónica', 'tecnología', 'Electricidad', 'generación de energía', 'Electrónica', 'automatización', 'Ingeniería de vehículos de motor'
			, 'Ingeniero de aeronaves', 'Ingeniero  de mecánica', 'Ingeniero electrónica', 'Ingeniero de tecnología', 'odontologo', 'Medico', 'Agronomo', 'veterinaro'
			, 'Ingeniería mecánica ', 'Ingeniería metalurgia', 'Ingeniería química', 'Filosofo','Historiador', 'Físico', 'Químico', 'Matemático', 'estadístico'
			, 'Docente', 'Docente para educación básica', 'Docente nivel preescolar', 'Biologo',  'bioquímico'
			, 'pescador', 'ganadero',"Maquinista naval","Astrónomo","Biotecnólogo","Químico industrial","Ingeniero bioquímico", "Ingeniero nuclear","Toxicólogo"
			, "Creativo publicitario","Operador de cámara","Community manager","Documentalista de radio y televisión","Presentador de radio y televisión"
			, "Vigilante de seguridad","Instructor de fitness", "Profesor de yoga", "Nutricionista", "Terapeuta deportivo", "Diseñador de páginas web"
			, "Asistente de diseñador", "Diseñador de moda", "Diseñador gráfico", "Técnico en reprografía", "Rotulista" ]
	
	
		Random rand = new Random()
		
		int ranlist2 = rand.nextInt(puesto.size())
	
		println(puesto.get(ranlist2))
	
		GlobalVariable.puesto = puesto.get(ranlist2)
	}
	
	def tiposPuestos () {
	tipoPuesto = ["40288086796be11e01796c18e54f0063","40288086796be11e01796c18e3040061","40288086796be11e01796c18e1e50060","40288086796be11e01796c18e7960065,40288086796be11e01796c18eaff0068",
		"2c9f84e1884991ea01885517de9e04dc","40288086796be11e01796c18e66f0064","40288086796be11e01796c18f0a7006d","40288086796be11e01796c18ec210069","40288086796be11e01796c18e0c7005f",
		"40288086796be11e01796c18ef83006c","40288086796be11e01796c18e4260062"]
	
	Random rand = new Random()
	
	int ranlist3 = rand.nextInt(tipoPuesto.size())
	
	println(tipoPuesto.get(ranlist3))
	
	GlobalVariable.tipoPuesto = tipoPuesto.get(ranlist3)
	}
	
	def academyNivel () {
		NivelAcademico = ["402881cf79c889e50179c88d84120000","402881cf79c889e50179c88e9ce20001","402881cf79c889e50179c88f42650002","402881cf79c889e50179c88fc5850003",
			"402881cf79c889e50179c88feecf0004","402881cf79c889e50179c8902b470005","402881cf79c889e50179c8903e100006"]
		
		Random rand = new Random()
		
		int ranlist4 = rand.nextInt(NivelAcademico.size())
		
		println(NivelAcademico.get(ranlist4))
		
		GlobalVariable.NivelAcademico = NivelAcademico.get(ranlist4)
	}