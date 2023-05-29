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


minimum = 1000
maximum = 30000
Salario()
salaryrequery = salaryrequery + 1000
println(salaryrequery)
GlobalVariable.SalarioDeseado = salaryrequery

experiencia()
experiencia = experiencia +1 as int
GlobalVariable.experiencia = experiencia
println("estos son los años " + GlobalVariable.experiencia)

println ("la fecha es")
date()

println ("el puesto es")
puestos()

println ("el tipo de puesto es")
tiposPuestos ()

println ("el nivel academico es")
academyNivel ()

def Salario() {
	salaryrequery= ((Math.random() * 30000) as int)
	}
	
def experiencia() {
	experiencia= ((Math.random() * 60) as int)
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

	puesto = ['Administración y gestión de empresas', 'Contabilidad y fiscalización', 'Finanzas", "banca y seguros', 'Negocios y administración, programas multidisciplinarios o generales'
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